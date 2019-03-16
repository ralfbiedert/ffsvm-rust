use std::{
    num::{ParseFloatError, ParseIntError},
    option::NoneError,
};

/// Possible error types when classifying with a [`SVMCore`](crate::SVMCore).
#[derive(Debug)]
pub enum Error {
    /// This can be emitted when creating a SVM from a [`ModelFile`](crate::ModelFile). For models generated by
    /// libSVM's `svm-train`, the most common reason this occurs is skipping attributes.
    /// All attributes must be in sequential order 0, 1, 2, ..., n. If they are not, this
    /// error will be emitted. For more details see the documentation provided in [`ModelFile`](crate::ModelFile).
    AttributesUnordered {
        /// The index process that was not a direct successor of the previous index. Can be used for
        /// easier debugging the model file.
        index: u32,

        /// The value of the given index. Can be used for debugging in conjunction with `index`.
        value: f32,

        /// The last index processed. If everything were alright, then `index` should equal
        /// `last_index + 1`.
        last_index: u32,
    },

    /// This error can be emitted by [`Predict::predict_probability`](crate::Predict::predict_probability) in case the model loaded by
    ///  [`ModelFile`](crate::ModelFile) was not trained with probability estimates (`svm-train -b 1`).
    NoProbabilities,

    /// Can be emitted by [`Predict::predict_probability`](crate::Predict::predict_probability) when predicting probabilities
    /// and the internal iteration limit was exceeded.
    IterationsExceeded,

    /// If the model does not have a `gamma` set this error may be raised.
    NoGamma,

    /// If the model does not have a `coef0` set this error may be raised.
    NoCoef0,

    /// If the model does not have a `degree` set this error may be raised.
    NoDegree,

    /// Wrapper for internal parsing error when unifiying error handling.
    ParsingError(String),
}

// impl<'a, T> From<Error<'a, T>> for Error {
//     fn from(_: Error<'a, T>) -> Self {
//         Error::ParsingError
//     }
// }


impl From<NoneError> for Error {
    fn from(_: NoneError) -> Self {
        Error::ParsingError("NoneError".to_owned())
    }
}

impl From<ParseFloatError> for Error {
    fn from(_e: ParseFloatError) -> Self {
        Error::ParsingError("ParseFloatError".to_owned())
    }
}

impl From<ParseIntError> for Error {
    fn from(_: ParseIntError) -> Self {
        Error::ParsingError("ParseIntError".to_owned())
    }
}
