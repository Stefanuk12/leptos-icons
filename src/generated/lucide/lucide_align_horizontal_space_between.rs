use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" height = "14" x = "3" y = "5" width = "6" ></ rect > < rect x = "15" y = "7" height = "10" width = "6" rx = "2" ></ rect > < path d = "M3 2v20" ></ path > < path d = "M21 2v20" ></ path > < / > } } pub const LUCIDE_ALIGN_HORIZONTAL_SPACE_BETWEEN : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-width" , "2") , ("width" , "24")] } ;