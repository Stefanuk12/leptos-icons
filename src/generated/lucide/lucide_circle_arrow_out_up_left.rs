use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 8V2h6" ></ path > < path d = "m2 2 10 10" ></ path > < path d = "M12 2A10 10 0 1 1 2 12" ></ path > < / > } } pub const LUCIDE_CIRCLE_ARROW_OUT_UP_LEFT : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("width" , "24")] } ;