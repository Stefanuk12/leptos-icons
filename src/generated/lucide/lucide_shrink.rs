use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m15 15 6 6m-6-6v4.8m0-4.8h4.8" ></ path > < path d = "M9 19.8V15m0 0H4.2M9 15l-6 6" ></ path > < path d = "M15 4.2V9m0 0h4.8M15 9l6-6" ></ path > < path d = "M9 4.2V9m0 0H4.2M9 9 3 3" ></ path > < / > } } pub const LUCIDE_SHRINK : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("fill" , "none") , ("height" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24")] } ;