use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M19 3H5" ></ path > < path d = "M12 21V7" ></ path > < path d = "m6 15 6 6 6-6" ></ path > < / > } } pub const LUCIDE_ARROW_DOWN_FROM_LINE : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("width" , "24") , ("height" , "24") , ("viewBox" , "0 0 24 24")] } ;