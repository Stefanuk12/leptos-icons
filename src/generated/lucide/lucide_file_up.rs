use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" ></ path > < path d = "M14 2v4a2 2 0 0 0 2 2h4" ></ path > < path d = "M12 12v6" ></ path > < path d = "m15 15-3-3-3 3" ></ path > < / > } } pub const LucideFileUp : Path = Path { path : icon_path , props : & [("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("height" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke-linejoin" , "round")] } ;