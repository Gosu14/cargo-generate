use std::fmt::Display;

use heck::{ToKebabCase, ToSnakeCase};

use crate::user_parsed_input::UserParsedInput;

use super::ProjectNameInput;

#[derive(Debug, PartialEq)]
pub struct ProjectName(String);

impl From<(&ProjectNameInput, &UserParsedInput)> for ProjectName {
    fn from(
        (project_name_input, user_parsed_input): (&ProjectNameInput, &UserParsedInput),
    ) -> Self {
        Self(if user_parsed_input.force() {
            project_name_input.as_ref().to_owned()
        } else {
            sanitize_project_name(project_name_input.as_ref())
        })
    }
}

impl AsRef<str> for ProjectName {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl Display for ProjectName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

pub fn sanitize_project_name(name: &str) -> String {
    let snake_case_project_name = name.to_snake_case();
    if snake_case_project_name == name {
        snake_case_project_name
    } else {
        name.to_kebab_case()
    }
}

#[cfg(test)]
mod tests {
    use crate::absolute_path::AbsolutePathExt;
    use std::path::Path;

    use super::*;
    use crate::user_parsed_input::UserParsedInputBuilder;

    #[test]
    fn ensure_that_absolute_path_of_non_existing_path_works() {
        #[cfg(target_family = "unix")]
        let path = Path::new("/non-existing-path");
        #[cfg(target_family = "windows")]
        let path = Path::new("D:\\non-existing-path");

        let path_absolutize = path.as_absolute().unwrap();
        assert_eq!(path_absolutize, path);
    }

    #[test]
    fn test_snake_case_is_accepted() {
        let input = ProjectNameInput("lock_firmware".to_string());
        let args = UserParsedInputBuilder::for_testing().build();

        let project_name = ProjectName::from((&input, &args));
        assert_eq!(project_name, ProjectName("lock_firmware".into()));
    }

    #[test]
    fn test_dash_case_is_accepted() {
        let input = ProjectNameInput("lock-firmware".to_string());
        let args = UserParsedInputBuilder::for_testing().build();

        let project_name = ProjectName::from((&input, &args));
        assert_eq!(project_name, ProjectName("lock-firmware".into()));
    }

    #[test]
    fn test_converted_to_dash_case() {
        let input = ProjectNameInput("lockFirmware".to_string());
        let args = UserParsedInputBuilder::for_testing().build();

        let project_name = ProjectName::from((&input, &args));
        assert_eq!(project_name, ProjectName("lock-firmware".into()));
    }

    #[test]
    fn test_not_converted_to_dash_case_when_with_force() {
        let input = ProjectNameInput("lockFirmware".to_string());
        let args = UserParsedInputBuilder::for_testing().with_force().build();

        let project_name = ProjectName::from((&input, &args));
        assert_eq!(project_name, ProjectName("lockFirmware".into()));
    }
}
