use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 20h20" ></ path > < path d = "m9 10 2 2 4-4" ></ path > < rect rx = "2" width = "18" y = "4" x = "3" height = "12" ></ rect > < / > } } pub const LUCIDE_LAPTOP_MINIMAL_CHECK : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;