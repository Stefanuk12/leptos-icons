use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "14" height = "6" x = "5" y = "15" rx = "2" ></ rect > < rect height = "6" x = "7" y = "3" width = "10" rx = "2" ></ rect > < path d = "M2 21h20" ></ path > < path d = "M2 3h20" ></ path > < / > } } pub const LUCIDE_ALIGN_VERTICAL_SPACE_BETWEEN : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-width" , "2") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-linecap" , "round")] } ;