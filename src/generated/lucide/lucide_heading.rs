use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M6 12h12" ></ path > < path d = "M6 20V4" ></ path > < path d = "M18 20V4" ></ path > < / > } } pub const LUCIDE_HEADING : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("height" , "24") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round")] } ;