use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 6V2H8" ></ path > < path d = "m8 18-4 4V8a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2Z" ></ path > < path d = "M2 12h2" ></ path > < path d = "M9 11v2" ></ path > < path d = "M15 11v2" ></ path > < path d = "M20 12h2" ></ path > < / > } } pub const LUCIDE_BOT_MESSAGE_SQUARE : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24")] } ;