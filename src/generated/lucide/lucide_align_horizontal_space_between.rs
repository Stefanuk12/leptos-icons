use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "6" y = "5" rx = "2" x = "3" height = "14" ></ rect > < rect width = "6" y = "7" rx = "2" height = "10" x = "15" ></ rect > < path d = "M3 2v20" ></ path > < path d = "M21 2v20" ></ path > < / > } } pub const LUCIDE_ALIGN_HORIZONTAL_SPACE_BETWEEN : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("height" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24")] } ;