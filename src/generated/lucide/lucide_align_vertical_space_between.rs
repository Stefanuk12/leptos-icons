use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "6" rx = "2" x = "5" y = "15" width = "14" ></ rect > < rect rx = "2" width = "10" x = "7" height = "6" y = "3" ></ rect > < path d = "M2 21h20" ></ path > < path d = "M2 3h20" ></ path > < / > } } pub const LUCIDE_ALIGN_VERTICAL_SPACE_BETWEEN : Path = Path { path : icon_path , props : & [("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24")] } ;