use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M13.228 21.925A10 10 0 1 1 21.994 12.338" ></ path > < path d = "M12 6v6l1.562.781" ></ path > < path d = "m14 18 4-4 4 4" ></ path > < path d = "M18 22v-8" ></ path > < / > } } pub const LUCIDE_CLOCK_ARROW_UP : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24")] } ;