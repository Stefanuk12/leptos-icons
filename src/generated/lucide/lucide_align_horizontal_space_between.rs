use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "6" y = "5" x = "3" rx = "2" height = "14" ></ rect > < rect height = "10" width = "6" y = "7" rx = "2" x = "15" ></ rect > < path d = "M3 2v20" ></ path > < path d = "M21 2v20" ></ path > < / > } } pub const LUCIDE_ALIGN_HORIZONTAL_SPACE_BETWEEN : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linecap" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round")] } ;