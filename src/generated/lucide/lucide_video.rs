use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m22 8-6 4 6 4V8Z" ></ path > < rect ry = "2" width = "14" x = "2" height = "12" y = "6" rx = "2" ></ rect > < / > } } pub const LucideVideo : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("width" , "24")] } ;