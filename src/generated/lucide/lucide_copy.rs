use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect ry = "2" width = "14" height = "14" x = "8" rx = "2" y = "8" ></ rect > < path d = "M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2" ></ path > < / > } } pub const LUCIDE_COPY : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("fill" , "none") , ("height" , "24") , ("viewBox" , "0 0 24 24")] } ;