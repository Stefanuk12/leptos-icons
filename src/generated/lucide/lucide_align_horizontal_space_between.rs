use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" rx = "2" height = "14" width = "6" y = "5" ></ rect > < rect width = "6" height = "10" y = "7" rx = "2" x = "15" ></ rect > < path d = "M3 2v20" ></ path > < path d = "M21 2v20" ></ path > < / > } } pub const LUCIDE_ALIGN_HORIZONTAL_SPACE_BETWEEN : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("width" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round")] } ;