use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "10" y = "7" width = "6" rx = "2" x = "9" ></ rect > < path d = "M4 22V2" ></ path > < path d = "M20 22V2" ></ path > < / > } } pub const LUCIDE_ALIGN_HORIZONTAL_SPACE_AROUND : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-linecap" , "round")] } ;