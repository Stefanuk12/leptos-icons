use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 17V3" ></ path > < path d = "m6 11 6 6 6-6" ></ path > < path d = "M19 21H5" ></ path > < / > } } pub const LUCIDE_ARROW_DOWN_TO_LINE : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("width" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("height" , "24")] } ;