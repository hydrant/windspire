pub trait UseCase<Response> {
    fn execute(&self) -> Response;
}
