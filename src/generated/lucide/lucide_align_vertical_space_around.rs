use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" x = "7" height = "6" y = "9" width = "10" ></ rect > < path d = "M22 20H2" ></ path > < path d = "M22 4H2" ></ path > < / > } } pub const LUCIDE_ALIGN_VERTICAL_SPACE_AROUND : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("height" , "24") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("fill" , "none")] } ;