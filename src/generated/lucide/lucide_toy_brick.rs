use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M10 8V5c0-.6-.4-1-1-1H6a1 1 0 0 0-1 1v3" ></ path > < path d = "M19 8V5c0-.6-.4-1-1-1h-3a1 1 0 0 0-1 1v3" ></ path > < / > } } pub const LucideToyBrick : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("width" , "24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round")] } ;