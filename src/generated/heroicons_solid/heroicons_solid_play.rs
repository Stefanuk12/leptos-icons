use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4.5 5.653c0-1.427 1.529-2.33 2.779-1.643l11.54 6.347c1.295.712 1.295 2.573 0 3.286L7.28 19.99c-1.25.687-2.779-.217-2.779-1.643V5.653Z" fill - rule = "evenodd" clip - rule = "evenodd" ></ path > < / > } } pub const HEROICONS_SOLID_PLAY : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "currentColor") , ("aria-hidden" , "true") , ("data-slot" , "icon")] } ;