use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M7 3.34V5a3 3 0 0 0 3 3" ></ path > < path d = "M11 21.95V18a2 2 0 0 0-2-2 2 2 0 0 1-2-2v-1a2 2 0 0 0-2-2H2.05" ></ path > < path d = "M21.54 15H17a2 2 0 0 0-2 2v4.54" ></ path > < path d = "M12 2a10 10 0 1 0 9.54 13" ></ path > < path d = "M20 6V4a2 2 0 1 0-4 0v2" ></ path > < rect width = "8" height = "5" x = "14" rx = "1" y = "6" ></ rect > < / > } } pub const LUCIDE_EARTH_LOCK : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("fill" , "none")] } ;