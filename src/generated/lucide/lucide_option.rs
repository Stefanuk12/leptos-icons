use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 3h6l6 18h6" ></ path > < path d = "M14 3h7" ></ path > < / > } } pub const LUCIDE_OPTION : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;