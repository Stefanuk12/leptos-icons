use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 8v11a2 2 0 0 0 2 2h2" ></ path > < path d = "M20 8v11a2 2 0 0 1-2 2h-2" ></ path > < path d = "m9 15 3-3 3 3" ></ path > < path d = "M12 12v9" ></ path > < / > } } pub const LucideArchiveRestore : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("height" , "24")] } ;