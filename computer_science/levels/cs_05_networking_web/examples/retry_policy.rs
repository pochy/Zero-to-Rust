#[derive(Debug, Clone, Copy)]
enum Operation {
    GetUser,
    CreateOrder,
    ChargeCard,
    SendEmail,
}

#[derive(Debug, Clone, Copy)]
enum Failure {
    Timeout,
    ConnectionReset,
    HttpStatus(u16),
}

fn can_retry(operation: Operation, failure: Failure) -> bool {
    match (operation, failure) {
        (Operation::GetUser, Failure::Timeout | Failure::ConnectionReset) => true,
        (Operation::SendEmail, Failure::Timeout | Failure::ConnectionReset) => true,
        (_, Failure::HttpStatus(status)) if status >= 500 => {
            matches!(operation, Operation::GetUser | Operation::SendEmail)
        }
        (Operation::CreateOrder | Operation::ChargeCard, _) => false,
        (_, Failure::HttpStatus(_)) => false,
    }
}

fn main() {
    let cases = [
        (Operation::GetUser, Failure::Timeout),
        (Operation::CreateOrder, Failure::Timeout),
        (Operation::ChargeCard, Failure::ConnectionReset),
        (Operation::SendEmail, Failure::HttpStatus(503)),
        (Operation::GetUser, Failure::HttpStatus(404)),
    ];

    for (operation, failure) in cases {
        println!(
            "{operation:?} after {failure:?}: retry={}",
            can_retry(operation, failure)
        );
    }
}
