use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 3v14" ></ path > < path d = "M5 10h14" ></ path > < path d = "M5 21h14" ></ path > < / > } } pub const LUCIDE_DIFF : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;