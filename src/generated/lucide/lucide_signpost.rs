use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 3v3" ></ path > < path d = "M18.5 13h-13L2 9.5 5.5 6h13L22 9.5Z" ></ path > < path d = "M12 13v8" ></ path > < / > } } pub const LUCIDE_SIGNPOST : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("height" , "24") , ("width" , "24") , ("fill" , "none")] } ;