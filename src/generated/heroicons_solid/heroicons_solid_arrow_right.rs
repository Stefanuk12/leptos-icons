use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12.97 3.97a.75.75 0 0 1 1.06 0l7.5 7.5a.75.75 0 0 1 0 1.06l-7.5 7.5a.75.75 0 1 1-1.06-1.06l6.22-6.22H3a.75.75 0 0 1 0-1.5h16.19l-6.22-6.22a.75.75 0 0 1 0-1.06Z" clip - rule = "evenodd" fill - rule = "evenodd" ></ path > < / > } } pub const HeroiconsSolidArrowRight : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("aria-hidden" , "true") , ("data-slot" , "icon") , ("fill" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;