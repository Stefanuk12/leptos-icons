use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 3v3" ></ path > < path d = "M18.5 13h-13L2 9.5 5.5 6h13L22 9.5Z" ></ path > < path d = "M12 13v8" ></ path > < / > } } pub const LUCIDE_SIGNPOST : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("width" , "24")] } ;