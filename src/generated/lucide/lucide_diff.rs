use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 3v14" ></ path > < path d = "M5 10h14" ></ path > < path d = "M5 21h14" ></ path > < / > } } pub const LUCIDE_DIFF : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor")] } ;