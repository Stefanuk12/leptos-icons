use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M15 3v18" ></ path > < rect height = "18" width = "18" rx = "2" x = "3" y = "3" ></ rect > < path d = "M21 9H3" ></ path > < path d = "M21 15H3" ></ path > < / > } } pub const LucideTableProperties : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24")] } ;