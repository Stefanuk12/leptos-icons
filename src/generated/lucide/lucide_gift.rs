use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "4" x = "3" rx = "1" width = "18" y = "8" ></ rect > < path d = "M12 8v13" ></ path > < path d = "M19 12v7a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2v-7" ></ path > < path d = "M7.5 8a2.5 2.5 0 0 1 0-5A4.8 8 0 0 1 12 8a4.8 8 0 0 1 4.5-5 2.5 2.5 0 0 1 0 5" ></ path > < / > } } pub const LUCIDE_GIFT : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("width" , "24") , ("height" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;