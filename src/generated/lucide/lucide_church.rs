use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m18 7 4 2v11a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V9l4-2" ></ path > < path d = "M14 22v-4a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v4" ></ path > < path d = "M18 22V5l-6-3-6 3v17" ></ path > < path d = "M12 7v5" ></ path > < path d = "M10 9h4" ></ path > < / > } } pub const LucideChurch : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("fill" , "none")] } ;