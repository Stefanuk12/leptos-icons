use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "13" x = "6" y = "4" width = "16" rx = "2" ></ rect > < path d = "m22 7-7.1 3.78c-.57.3-1.23.3-1.8 0L6 7" ></ path > < path d = "M2 8v11c0 1.1.9 2 2 2h14" ></ path > < / > } } pub const LUCIDE_MAILS : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("height" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2")] } ;