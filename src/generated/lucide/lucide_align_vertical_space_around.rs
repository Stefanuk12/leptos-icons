use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "9" height = "6" width = "10" rx = "2" x = "7" ></ rect > < path d = "M22 20H2" ></ path > < path d = "M22 4H2" ></ path > < / > } } pub const LUCIDE_ALIGN_VERTICAL_SPACE_AROUND : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("width" , "24")] } ;