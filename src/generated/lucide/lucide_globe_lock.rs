use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M15.686 15A14.5 14.5 0 0 1 12 22a14.5 14.5 0 0 1 0-20 10 10 0 1 0 9.542 13" ></ path > < path d = "M2 12h8.5" ></ path > < path d = "M20 6V4a2 2 0 1 0-4 0v2" ></ path > < rect height = "5" width = "8" y = "6" rx = "1" x = "14" ></ rect > < / > } } pub const LUCIDE_GLOBE_LOCK : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round")] } ;