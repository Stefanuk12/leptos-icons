use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "14" height = "6" rx = "2" y = "15" x = "5" ></ rect > < rect width = "10" x = "7" y = "3" height = "6" rx = "2" ></ rect > < path d = "M2 21h20" ></ path > < path d = "M2 3h20" ></ path > < / > } } pub const LUCIDE_ALIGN_VERTICAL_SPACE_BETWEEN : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;