use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 3v18" ></ path > < rect height = "18" width = "18" x = "3" y = "3" rx = "2" ></ rect > < path d = "M3 9h18" ></ path > < path d = "M3 15h18" ></ path > < / > } } pub const LucideTable : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("fill" , "none")] } ;