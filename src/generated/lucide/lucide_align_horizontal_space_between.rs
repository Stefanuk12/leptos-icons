use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" width = "6" rx = "2" height = "14" y = "5" ></ rect > < rect y = "7" rx = "2" x = "15" height = "10" width = "6" ></ rect > < path d = "M3 2v20" ></ path > < path d = "M21 2v20" ></ path > < / > } } pub const LUCIDE_ALIGN_HORIZONTAL_SPACE_BETWEEN : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;