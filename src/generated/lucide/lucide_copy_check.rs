use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m12 15 2 2 4-4" ></ path > < rect ry = "2" x = "8" width = "14" y = "8" rx = "2" height = "14" ></ rect > < path d = "M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2" ></ path > < / > } } pub const LUCIDE_COPY_CHECK : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24")] } ;