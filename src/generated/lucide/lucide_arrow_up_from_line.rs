use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m18 9-6-6-6 6" ></ path > < path d = "M12 3v14" ></ path > < path d = "M5 21h14" ></ path > < / > } } pub const LUCIDE_ARROW_UP_FROM_LINE : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke-linejoin" , "round") , ("height" , "24")] } ;