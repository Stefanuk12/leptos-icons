use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "12" r = "10" ></ circle > < path d = "M12 8v8" ></ path > < path d = "m8 12 4 4 4-4" ></ path > < / > } } pub const LUCIDE_CIRCLE_ARROW_DOWN : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor")] } ;