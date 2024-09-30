
use ic_cdk_macros::update;

use crate::{CarStatus, PaymentStatus, RentalTransaction, STATE};
#[update]
fn reserve_car(car_id: u64, start_timestamp: u128, end_timestamp: u128) -> Result<RentalTransaction, String> {
    if start_timestamp >= end_timestamp {
        return Err("Invalid time range".to_string());
    }
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        if let Some(car) = state.cars.get_mut(&car_id) {
            match car.get_booking_status_at_give_time_period(start_timestamp, end_timestamp) {
                CarStatus::Available => {
                    let customer_id = ic_cdk::caller(); 
                   
                    let total_days = (end_timestamp - start_timestamp) as f64 / 86400.0; 
                    let total_amount = car.details.current_price_per_day * total_days as f64;  
                    let transaction = RentalTransaction {
                        car_principal_id: car_id,
                        customer_principal_id: customer_id,
                        customer_name: "Customer Name".to_string(),
                        start_timestamp,
                        end_timestamp,
                        total_amount,
                        payment_status: PaymentStatus::Unpaid,
                    };
                    car.bookings.push(transaction.clone());
                    return Ok(transaction);
                },
                _ => return Err("Car is not available".to_string()),
            }
        }
        Err("Car not found".to_string())
    })
}
