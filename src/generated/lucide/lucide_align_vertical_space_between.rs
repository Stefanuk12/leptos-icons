use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "14" x = "5" rx = "2" height = "6" y = "15" ></ rect > < rect rx = "2" height = "6" y = "3" x = "7" width = "10" ></ rect > < path d = "M2 21h20" ></ path > < path d = "M2 3h20" ></ path > < / > } } pub const LUCIDE_ALIGN_VERTICAL_SPACE_BETWEEN : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;