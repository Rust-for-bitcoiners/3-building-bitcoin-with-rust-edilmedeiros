#![allow(unused)]

#[derive(PartialEq, Debug)]
enum MResult<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> MResult<T, E> {
    fn ok(value: T) -> Self {
        MResult::Ok(value)
    }

    // Function to create an Err variant
    fn err(error: E) -> Self {
        MResult::Err(error)
    }

    // Method to check if it's an Ok variant
    fn is_ok(&self) -> bool {
        match self {
            MResult::Ok(_)  => true,
            MResult::Err(_) => false,
        }
    }

    // Method to check if it's an Err variant
    fn is_err(&self) -> bool {
        match self {
            MResult::Ok(_)  => false,
            MResult::Err(_) => true,
        }
    }

    // Method to unwrap the Ok value, panics if it's an Err
    fn unwrap(self) -> T {
        match self {
            MResult::Ok(x)  => x,
            MResult::Err(_) => panic!(),
        }
    }

    // Method to unwrap the Err value, panics if it's an Ok
    fn unwrap_err(self) -> E {
        match self {
            MResult::Ok(_)  => panic!(),
            MResult::Err(e) => e,
        }

    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_ok() {
        assert_eq!(MResult::<i32, i32>::ok(0), MResult::Ok(0));
    }

    #[test]
    fn test_err() {
        assert_eq!(MResult::<i32, i32>::err(0), MResult::Err(0));
    }

    #[test]
    fn test_is_ok() {
        let ok = MResult::<i32, i32>::Ok(0);
        assert!(ok.is_ok());

        let err = MResult::<i32, i32>::Err(0);
        assert!(!err.is_ok());
    }

    #[test]
    fn test_is_err() {
        let ok = MResult::<i32, i32>::Ok(0);
        assert!(!ok.is_err());

        let err = MResult::<i32, i32>::Err(0);
        assert!(err.is_err());
    }

    #[test]
    fn test_unwrap() {
        let ok = MResult::<i32, i32>::Ok(0);
        assert_eq!(ok.unwrap(), 0);

        // The following should panic
        let err = MResult::<i32, i32>::Err(0);
        let result = std::panic::catch_unwind(|| err.unwrap());
        assert!(result.is_err());
    }

    #[test]
    fn test_unwrap_err() {
        // First case should panic
        let ok = MResult::<i32, i32>::Ok(0);
        let result = std::panic::catch_unwind(|| ok.unwrap_err());
        assert!(result.is_err());

        let err = MResult::<i32, i32>::Err(0);
        assert_eq!(err.unwrap_err(), 0);
    }


}
