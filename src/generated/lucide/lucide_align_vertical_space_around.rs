use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "6" rx = "2" x = "7" width = "10" y = "9" ></ rect > < path d = "M22 20H2" ></ path > < path d = "M22 4H2" ></ path > < / > } } pub const LUCIDE_ALIGN_VERTICAL_SPACE_AROUND : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("height" , "24")] } ;