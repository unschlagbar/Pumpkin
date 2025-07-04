use async_trait::async_trait;
use pumpkin_protocol::java::client::play::{ArgumentType, CommandSuggestion, SuggestionProviders};
use pumpkin_util::math::vector2::Vector2;
use pumpkin_util::math::vector3::Vector3;

use crate::command::CommandSender;
use crate::command::dispatcher::CommandError;
use crate::command::tree::RawArgs;
use crate::server::Server;

use super::super::args::ArgumentConsumer;
use super::coordinate::MaybeRelativeCoordinate;
use super::{Arg, DefaultNameArgConsumer, FindArg, GetClientSideArgParser};

/// x and z coordinates only
///
/// todo: implement ~ ^ notations
pub struct Position2DArgumentConsumer;

impl GetClientSideArgParser for Position2DArgumentConsumer {
    fn get_client_side_parser(&self) -> ArgumentType {
        ArgumentType::Vec2
    }

    fn get_client_side_suggestion_type_override(&self) -> Option<SuggestionProviders> {
        None
    }
}

#[async_trait]
impl ArgumentConsumer for Position2DArgumentConsumer {
    async fn consume<'a>(
        &'a self,
        src: &CommandSender,
        _server: &'a Server,
        args: &mut RawArgs<'a>,
    ) -> Option<Arg<'a>> {
        let pos = MaybeRelativePosition2D::try_new(args.pop()?, args.pop()?)?;

        let vec2 = pos.try_to_absolute(src.position())?;

        Some(Arg::Pos2D(vec2))
    }

    async fn suggest<'a>(
        &'a self,
        _sender: &CommandSender,
        _server: &'a Server,
        _input: &'a str,
    ) -> Result<Option<Vec<CommandSuggestion>>, CommandError> {
        Ok(None)
    }
}

struct MaybeRelativePosition2D(
    MaybeRelativeCoordinate<false>,
    MaybeRelativeCoordinate<false>,
);

impl MaybeRelativePosition2D {
    fn try_new(x: &str, z: &str) -> Option<Self> {
        Some(Self(x.try_into().ok()?, z.try_into().ok()?))
    }

    fn try_to_absolute(self, origin: Option<Vector3<f64>>) -> Option<Vector2<f64>> {
        Some(Vector2::new(
            self.0.into_absolute(origin.map(|o| o.x))?,
            self.1.into_absolute(origin.map(|o| o.z))?,
        ))
    }
}

impl DefaultNameArgConsumer for Position2DArgumentConsumer {
    fn default_name(&self) -> &'static str {
        "pos"
    }
}

impl<'a> FindArg<'a> for Position2DArgumentConsumer {
    type Data = Vector2<f64>;

    fn find_arg(args: &'a super::ConsumedArgs, name: &str) -> Result<Self::Data, CommandError> {
        match args.get(name) {
            Some(Arg::Pos2D(data)) => Ok(*data),
            _ => Err(CommandError::InvalidConsumption(Some(name.to_string()))),
        }
    }
}
