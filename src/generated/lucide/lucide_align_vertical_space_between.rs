use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "14" height = "6" rx = "2" x = "5" y = "15" ></ rect > < rect rx = "2" x = "7" height = "6" width = "10" y = "3" ></ rect > < path d = "M2 21h20" ></ path > < path d = "M2 3h20" ></ path > < / > } } pub const LUCIDE_ALIGN_VERTICAL_SPACE_BETWEEN : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2")] } ;