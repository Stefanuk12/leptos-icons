use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "22" y1 = "12" y2 = "12" x2 = "2" ></ line > < path d = "M5.45 5.11 2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z" ></ path > < line x2 = "6.01" x1 = "6" y1 = "16" y2 = "16" ></ line > < line y1 = "16" x1 = "10" x2 = "10.01" y2 = "16" ></ line > < / > } } pub const LUCIDE_HARD_DRIVE : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("width" , "24")] } ;