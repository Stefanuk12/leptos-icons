use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 8V2h6" ></ path > < path d = "m2 2 10 10" ></ path > < path d = "M12 2A10 10 0 1 1 2 12" ></ path > < / > } } pub const LUCIDE_ARROW_UP_LEFT_FROM_CIRCLE : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("height" , "24") , ("fill" , "none")] } ;