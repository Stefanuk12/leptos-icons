use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 5a3 3 0 1 0-5.997.125 4 4 0 0 0-2.526 5.77 4 4 0 0 0 .556 6.588A4 4 0 1 0 12 18Z" ></ path > < path d = "M9 13a4.5 4.5 0 0 0 3-4" ></ path > < path d = "M6.003 5.125A3 3 0 0 0 6.401 6.5" ></ path > < path d = "M3.477 10.896a4 4 0 0 1 .585-.396" ></ path > < path d = "M6 18a4 4 0 0 1-1.967-.516" ></ path > < path d = "M12 13h4" ></ path > < path d = "M12 18h6a2 2 0 0 1 2 2v1" ></ path > < path d = "M12 8h8" ></ path > < path d = "M16 8V5a2 2 0 0 1 2-2" ></ path > < circle r = ".5" cx = "16" cy = "13" ></ circle > < circle cy = "3" r = ".5" cx = "18" ></ circle > < circle r = ".5" cx = "20" cy = "21" ></ circle > < circle r = ".5" cx = "20" cy = "8" ></ circle > < / > } } pub const LUCIDE_BRAIN_CIRCUIT : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round")] } ;