use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M15.686 15A14.5 14.5 0 0 1 12 22a14.5 14.5 0 0 1 0-20 10 10 0 1 0 9.542 13" ></ path > < path d = "M2 12h8.5" ></ path > < path d = "M20 6V4a2 2 0 1 0-4 0v2" ></ path > < rect y = "6" width = "8" height = "5" x = "14" rx = "1" ></ rect > < / > } } pub const LUCIDE_GLOBE_LOCK : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke" , "currentColor") , ("height" , "24") , ("width" , "24")] } ;