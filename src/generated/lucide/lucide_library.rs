use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m16 6 4 14" ></ path > < path d = "M12 6v14" ></ path > < path d = "M8 8v12" ></ path > < path d = "M4 4v16" ></ path > < / > } } pub const LucideLibrary : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round")] } ;