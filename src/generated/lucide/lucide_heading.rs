use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M6 12h12" ></ path > < path d = "M6 20V4" ></ path > < path d = "M18 20V4" ></ path > < / > } } pub const LUCIDE_HEADING : Path = Path { path : icon_path , props : & [("fill" , "none") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;